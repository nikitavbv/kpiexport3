use std::collections::HashMap;
use serde::Deserialize;
use crate::errors::RozkladParseError;
use crate::models::schedule::*;

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
                entries.push(
                    GroupScheduleEntry::new(
                        ScheduleWeek::from_api_index(week.0.parse()?),
                        ScheduleDay::from_api_index(day.0.parse()?),
                        lesson.lesson_number.parse().map(|v: u8| v - 1)?
                    )
                        .with_names(vec![lesson.lesson_name])
                        .with_lecturers(lesson.teachers.iter().map(|v| v.teacher_short_name.clone()).collect())
                        .with_locations(lesson.rooms.iter().map(|v| v.room_name.clone()).collect())
                );
            }
        }
    }

    Ok(GroupSchedule { entries, source: Some(GroupScheduleSource::API) })
}

// rozklad api test
#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::assert_gt;

    #[tokio::test]
    async fn rozklad_group_schedule_ip82() {
        assert_gt!(group_schedule(&reqwest::Client::new(), "ІП-82").await.unwrap().entries.len(), 0);
    }
}