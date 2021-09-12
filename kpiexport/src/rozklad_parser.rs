use std::{collections::HashMap};
use scraper::{Html, Selector, ElementRef};
use serde::Deserialize;
use crate::models::schedule::*;
use crate::errors::RozkladParseError;
use crate::utils::group_id_from_url;

const USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.92 Safari/537.36";
const GROUP_PREFIXES: &'static [&'static str] = &[
    "А", "Б", "В", "Г", "Ґ", "Д", "Е", "Є", "Ж", "З", "И", "І", "Ї", "Й", "К", "Л", "М", "Н", "О", "П", "Р",
    "С", "Т", "У", "Ф", "Х", "Ц", "Ч", "Ш", "Щ", "Ю", "Я"
];

#[derive(Debug)]
struct GroupSelectionPageFormData {
    // naming kept same to original form
    viewstate: String,
    eventvalidation: String,
}

#[derive(Debug)]
pub struct GroupSchedulePageFormData {
    // naming kept same to original form
    viewstate: String,
    eventvalidation: String,
}

#[derive(Deserialize)]
struct GetGroupsResponse {
    d: Vec<String>
}

// get all groups
pub async fn get_groups(client: &reqwest::Client) -> Vec<String> {
    let mut groups = vec![];

    for prefix in GROUP_PREFIXES {
        match get_groups_with_prefix(&client, prefix).await {
            Ok(v) => groups.append(&mut v.clone()),
            Err(err) => {
                warn!("failed to get groups for prefix: {}", err);
                continue;
            }
        };
    }

    groups
}

async fn get_groups_with_prefix(client: &reqwest::Client, prefix: &str) -> Result<Vec<String>, RozkladParseError> {
    let res = client.post("http://rozklad.kpi.ua/Schedules/ScheduleGroupSelection.aspx/GetGroups")
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json; charset=UTF-8")
        .body(format!(r#"{{"prefixText":"{}","count":1000}}"#, prefix))
        .send()
        .await?;

    if res.status() != 200 {
        warn!("get groups response: {}", res.text().await?);
        return Err(RozkladParseError::RozkladErrored)
    }

    let response: GetGroupsResponse = res.json().await?;

    Ok(response.d)
}

// get schedule by group id
pub async fn group_schedule(client: &reqwest::Client, id: &str) -> Result<(GroupSchedule, GroupSchedulePageFormData), RozkladParseError> {
    let res = client.get(&format!("http://rozklad.kpi.ua/Schedules/ViewSchedule.aspx?g={}", id))
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;

    group_schedule_from_html(&res.text().await?)
}

pub async fn group_schedule_second_term(client: &reqwest::Client, id: &str) -> Result<GroupSchedule, RozkladParseError> {
    let first_term_reply = group_schedule(&client, &id).await?;

    let res = client.post(&format!("http://rozklad.kpi.ua/Schedules/ViewSchedule.aspx?g={}", id))
        .header("User-Agent", USER_AGENT)
        .form(&make_params_for_second_term_fetch(&first_term_reply.1))
        .send()
        .await?;

    group_schedule_from_html(&res.text().await?).map(|v| v.0)
}

fn group_schedule_from_html(html: &str) -> Result<(GroupSchedule, GroupSchedulePageFormData), RozkladParseError> {
    let document = Html::parse_document(&html);
    let mut entries = parse_week(
        &make_selector_and_select(&document, "#ctl00_MainContent_FirstScheduleTable>tbody")?,
        ScheduleWeek::First
    );
    entries.append(&mut parse_week(
        &make_selector_and_select(&document, "#ctl00_MainContent_SecondScheduleTable>tbody")?,
        ScheduleWeek::Second
    ));

    let viewstate = get_input_value(&make_selector_and_select(&document, "#__VIEWSTATE")?)?;
    let eventvalidation = get_input_value(&make_selector_and_select(&document, "#__EVENTVALIDATION")?)?;

    Ok((
        GroupSchedule { entries, source: GroupScheduleSource::Parser },
        GroupSchedulePageFormData {
            viewstate,
            eventvalidation
        }
    ))
}

fn parse_week(week_table: &ElementRef, week: ScheduleWeek) -> Vec<GroupScheduleEntry> {
    let mut entries = vec![];

    for row in week_table.children() {
        if ElementRef::wrap(row) == None {
            continue;
        }

        let mut index: Option<u8> = None;
        let mut day: ScheduleDay = ScheduleDay::Monday;

        for td in row.children() {
            let element = match ElementRef::wrap(td) {
                Some(v) => v,
                None => continue
            };

            let inner_html = element.inner_html();
            if inner_html.find("<br>") == Some(1) {
                index = inner_html[..1].parse().ok().map(|v: u8| v - 1);
                continue;
            }

            if index == None {
                continue;
            }

            if inner_html == "" {
                day = day.next();
                continue;
            }

            let mut names: Vec<String> = vec![];
            let mut lecturers: Vec<String> = vec![];
            let mut locations: Vec<String> = vec![];

            let mut names_parsed = false;
            let mut lecturers_parsed = false;

            for child in element.children() {
                let element = match ElementRef::wrap(child) {
                    Some(v) => v,
                    None => continue
                };

                let name = element.value().name();
                let inner_html = element.inner_html();

                if name == "span" {
                    for name_child in child.children() {
                        if let Some(element) = ElementRef::wrap(name_child) {
                            names.push(element.inner_html());
                        }
                    }
                    continue
                }

                if name == "br" {
                    if !names_parsed {
                        names_parsed = true;
                    } else if !lecturers_parsed {
                        lecturers_parsed = true;
                    }
                    continue;
                }

                if name != "a" {
                    continue;
                }

                if !lecturers_parsed && names_parsed {
                    lecturers.push(inner_html);
                } else if lecturers_parsed {
                    locations.push(inner_html);
                }
            }

            if index.is_some() {
                entries.push(GroupScheduleEntry {
                    week: week.clone(),
                    day: day.clone(),
                    index: index.unwrap(),
                    names,
                    lecturers,
                    locations
                });
            }

            day = day.next();
        }
    }

    entries
}

// get group id by name
pub async fn group_id_by_name(client: &reqwest::Client, name: &str) -> Result<String, RozkladParseError> {
    let group_selection_form_data = group_selection_page_form_data(&client).await?;

    let res = client.post("http://rozklad.kpi.ua/Schedules/ScheduleGroupSelection.aspx")
        .header("User-Agent", USER_AGENT)
        .form(&make_params(&group_selection_form_data, name))
        .send()
        .await?;

    group_id_from_url(&res.url().to_string())
}

async fn group_selection_page_form_data(client: &reqwest::Client) -> Result<GroupSelectionPageFormData, RozkladParseError> {
    let res = client.get("http://rozklad.kpi.ua/Schedules/ScheduleGroupSelection.aspx")
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;

    let document_text = res.text().await?;

    let document = Html::parse_document(&document_text);
    
    let viewstate = get_input_value(&make_selector_and_select(&document, "#__VIEWSTATE")?)?;
    let eventvalidation = get_input_value(&make_selector_and_select(&document, "#__EVENTVALIDATION")?)?;

    Ok(GroupSelectionPageFormData {
        viewstate,
        eventvalidation
    })
}

fn make_params(form_data: &GroupSelectionPageFormData, group_name: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    
    params.insert("__VIEWSTATE".to_string(), form_data.viewstate.clone());
    params.insert("__EVENTTARGET".into(), "".into());

    params.insert("__EVENTARGUMENT".into(), "".into());
    params.insert("ctl00$MainContent$ctl00$txtboxGroup".into(), group_name.into());
    params.insert("ctl00$MainContent$ctl00$btnShowSchedule".into(), "Розклад занять".into());
    params.insert("__EVENTVALIDATION".into(), form_data.eventvalidation.clone());

    params
}

fn make_params_for_second_term_fetch(form_data: &GroupSchedulePageFormData) -> HashMap<String, String> {
    let mut params = HashMap::new();

    params.insert("ctl00_ToolkitScriptManager_HiddenField".into(), "".into());

    params.insert("__VIEWSTATE".to_string(), form_data.viewstate.clone());
    params.insert("_EVENTTARGET".into(), "ctl00$MainContent$ddlSemesterType".into());
    params.insert("ctl00$MainContent$ddlSemesterType".into(), "2".into()); // for first term, this param is not set at all

    params.insert("__EVENTARGUMENT".into(), "".into());
    params.insert("__EVENTVALIDATION".into(), form_data.eventvalidation.clone());

    params
}

// html parsing
fn make_selector_and_select<'a>(document: &'a Html, selector: &str) -> Result<ElementRef<'a>, RozkladParseError> {
    select_first(&document, &make_selector(selector)?)
}

fn make_selector(selector: &str) -> Result<Selector, RozkladParseError> {
    Selector::parse(selector)
        .map_err(|err| RozkladParseError::HtmlParseFailed {
            description: format!("{:?}", err)
        })
}

fn select_first<'a>(document: &'a Html, selector: &Selector) -> Result<ElementRef<'a>, RozkladParseError> {
    document.select(&selector).into_iter().next()
        .ok_or(RozkladParseError::HtmlParseFailed {
            description: "failed to find element by selector".to_string()
        })
}

fn get_input_value(input: &ElementRef) -> Result<String, RozkladParseError> {
    get_attribute_value(&input, "value")
}

fn get_attribute_value(element: &ElementRef, attr_name: &str) -> Result<String, RozkladParseError> {
    element.value().attr(attr_name)
        .ok_or(RozkladParseError::HtmlParseFailed { description: "failed to get attribute value".to_string() })
        .map(|v| v.to_string())
}

// rozklad parser test
#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::assert_gt;

    #[tokio::test]
    async fn rozklad_get_id_ip82() {
        let ip82_group_id = group_id_by_name(&reqwest::Client::new(), "ІП-82").await.unwrap();
        let ip81_group_id = group_id_by_name(&reqwest::Client::new(), "ІП-81").await.unwrap();

        assert_eq!(ip82_group_id.len(), 36);
        assert_ne!(ip82_group_id, ip81_group_id);
    }

    #[tokio::test]
    async fn rozklad_schedule_ip82() {
        let ip82_group_id = group_id_by_name(&reqwest::Client::new(), "ІП-82").await.unwrap();
        assert_gt!(group_schedule(&reqwest::Client::new(), &ip82_group_id).await.unwrap().0.entries.len(), 0);
    }

    #[tokio::test]
    async fn rozklad_schedule_ip82_second_term() {
        println!("result is {:?}", group_schedule_second_term(&reqwest::Client::new(), "494e5743-35fb-4a3f-b868-44662e6cd66e").await.unwrap().entries);
    }

    #[tokio::test]
    async fn rozklad_groups() {
        assert_gt!(get_groups_with_prefix(&reqwest::Client::new(), "І").await.unwrap().len(), 0);
    }
}
