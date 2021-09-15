use std::collections::HashMap;
use serde::Deserialize;
use crate::errors::RozkladParseError;
use crate::models::schedule::*;
use crate::utils::group_id_from_url;

#[derive(Deserialize, Debug)]
struct GroupInfoResult {
    data: GroupInfoResultData
}

#[derive(Deserialize, Debug)]
struct GroupInfoResultData {
    group_url: String
}

#[derive(Deserialize, Debug)]
struct GroupTimetableResult {
    data: GroupTimetableResultData,
}

#[derive(Deserialize, Debug)]
struct GroupTimetableResultData {
    weeks: HashMap<String, GroupTimetableWeek>,
}

#[derive(Deserialize, Debug)]
struct GroupTimetableWeek {
    week_number: u8,
    days: HashMap<String, GroupTimetableDay>,
}

#[derive(Deserialize, Debug)]
struct GroupTimetableDay {
    day_number: u8,
    lessons: Vec<GroupTimetableLesson>
}

#[derive(Deserialize, Debug)]
struct GroupTimetableLesson {
    day_number: String,
    lesson_name: String,
    lesson_number: String,
    rooms: Vec<LessonRoom>,
    teachers: Vec<LessonTeacher>
}

#[derive(Deserialize, Debug)]
struct LessonRoom {
    room_name: String,
}

#[derive(Deserialize, Debug)]
struct LessonTeacher {
    teacher_short_name: String,
}

// get schedule by group id
pub async fn group_schedule(client: &reqwest::Client, name: &str) -> Result<GroupSchedule, RozkladParseError> {
    let res = client.get(&format!("https://api.rozklad.org.ua/v2/groups/{}/timetable", name))
        .send()
        .await?;

    if res.status() != 200 {
        return Err(RozkladParseError::RozkladApiErrored);
    }

    let res: GroupTimetableResult = res.json().await?;

    let mut entries = vec![];

    for week in res.data.weeks {
        for day in week.1.days {
            for lesson in day.1.lessons {
                entries.push(GroupScheduleEntry {
                    week: ScheduleWeek::from_api_index(week.0.parse()?),
                    day: ScheduleDay::from_api_index(day.0.parse()?),
                    index: lesson.lesson_number.parse().map(|v: u8| v - 1)?,
                    names: vec![ lesson.lesson_name ],
                    lecturers: lesson.teachers.iter().map(|v| v.teacher_short_name.clone()).collect(),
                    locations: lesson.rooms.iter().map(|v| v.room_name.clone()).collect(),
                })
            }
        }
    }

    Ok(GroupSchedule { entries, source: GroupScheduleSource::API })
}

// get group id by name
// for some reason this id seems to change every year, so it will not match rozklad if api is for prev year.
pub async fn group_id_by_name(client: &reqwest::Client, name: &str) -> Result<String, RozkladParseError> {
    let res = client.get(&format!("https://api.rozklad.org.ua/v2/groups/{}", name))
        .send()
        .await?;

    if res.status() != 200 {
        return Err(RozkladParseError::RozkladApiErrored);
    }

    let res: GroupInfoResult = res.json().await?;

    group_id_from_url(&res.data.group_url)
}

// rozklad api test
/*#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::assert_gt;

    #[tokio::test]
    async fn rozklad_get_id_ip82() {
        // note that id does not match the one scrapped. - update Jan 2021 - why?
        // Changed again in September 2021 - as far as I understand it updates with a delay after rozklad update.
        assert_eq!(group_id_by_name(&reqwest::Client::new(), "ІП-82").await.unwrap(), "92316a0c-5da0-496a-8fd7-378d3c78cc2d");
    }

    #[tokio::test]
    async fn rozklad_group_schedule_ip82() {
        assert_gt!(group_schedule(&reqwest::Client::new(), "ІП-82").await.unwrap().entries.len(), 0);
    }
}*/