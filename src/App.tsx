import React, {useState} from 'react';
import {
    AuthIntroScreen,
    ErrorMessage,
    ExportFinishedScreen,
    ExportInProgressScreen,
    ExportInputScreen
} from './components';

import './app.css';
import {
    Screen,
    CalendarEntry,
    CreateCalendarResponse,
    GetScheduleResponse,
    GoogleOAuthToken,
    GroupScheduleEntry,
} from './types';
import {oauthClientId} from './constants';
import moment from 'moment';

export const App = () => {
    const [error, setError] = useState<string|undefined>(undefined);
    const [screen, setScreen] = useState<Screen>('input');

    const [selectedGroup, setSelectedGroup] = useState<string>('');
    const [selectedCalendarName, setSelectedCalendarName] = useState<string>('');
    const [selectedStudentName, setSelectedStudentName] = useState<string>('');

    const [progressCurrent, setProgressCurrent] = useState<number>(0);
    const [progressTotal, setProgressTotal] = useState<number>(0);

    const updateProgress = (current: number, total: number) => {
        setProgressCurrent(current);
        setProgressTotal(total);
    };

    const exportScheduleFn = exportSchedule(setScreen, updateProgress);

    return (
        <>
            <main>
                <h1>KPI Exporter</h1>

                { error !== undefined ? <ErrorMessage errorText={error} /> : undefined }
                { screenElementByType(
                    screen,
                    progressCurrent,
                    progressTotal,
                    setScreen,
                    async (groupName: string, calendarName: string, studentName: string) => {
                        setSelectedGroup(groupName);
                        setSelectedCalendarName(calendarName);
                        setSelectedStudentName(studentName);
                        await exportScheduleFn(groupName, calendarName, studentName);
                    },
                    () => exportScheduleFn(selectedGroup, selectedCalendarName, selectedStudentName)
                ) }
            </main>
            <footer>
                by <a href="https://nikitavbv.com">nikitavbv</a>, see <a href="https://github.com/nikitavbv/kpiexport3">Github</a> for source code
            </footer>
        </>
    );
};

const screenElementByType = (type: Screen,
                             progressCurrent: number,
                             progressTotal: number,
                             setScreen: (s: Screen) => void,
                             onExportParamsSelected: (groupName: string, calendarName: string, studentName: string) => void,
                             onAuthIntroDone: () => void) => {
    switch (type)
    {
        case 'input':
            return (<ExportInputScreen onSubmit={onExportParamsSelected} />);
        case 'in_progress':
            return (<ExportInProgressScreen progressCurrent={progressCurrent} progressTotal={progressTotal} />);
        case 'finished':
            return (<ExportFinishedScreen setScreen={setScreen} />);
        case 'auth_intro':
            return (<AuthIntroScreen onDone={onAuthIntroDone} />);
    }
};

const exportSchedule = (setScreen: (s: Screen) => void, updateProgress: (progress: number, total: number) => void) =>
    async (groupName: string, calendarName: string, studentName: string) => {
    if (localStorage.authDone === undefined) {
        localStorage.authDone = true;
        setScreen('auth_intro');
        return;
    }

    const token = await get_google_token();
    const schedule = await scheduleForGroup(groupName, studentName);

    const progressTotal = schedule.entries.length + 1;
    let progressCounter = 0;
    const updateCurrentProgress = () => updateProgress(progressCounter, progressTotal);

    updateCurrentProgress();
    setScreen('in_progress');

    const calendar = await create_calendar(token, calendarName);
    progressCounter++;
    updateCurrentProgress();

    updateProgress(1, schedule.entries.length + 1);

    const allRequests = schedule.entries.map(entry =>
        fetch(`https://www.googleapis.com/calendar/v3/calendars/${calendar.id}/events?access_token=${token}`, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(create_calendar_event(entry))
        }).then(() => {
            progressCounter++;
            updateCurrentProgress();
        })
    );

    console.log('waiting for all events to be created');
    await Promise.all(allRequests);
    console.log('all events are created!');

    setScreen('finished');
};

const create_calendar_event = (entry: GroupScheduleEntry): CalendarEntry => {
    const m = moment();
    const isSecondSemester: boolean = m.month() < 6;
    const firstStudyMonth = !isSecondSemester ? 8 : 1;
    //let firstStudyDay = !isSecondSemester ? 1 : moment([m.year(), firstStudyMonth, 1, 0, 0]).day(8).date();
    const firstStudyDay = 1;
    const date = moment([m.year(), firstStudyMonth, firstStudyDay, 0, 0]).day(entry.day + 1);

    // shift the date of first course day by a week for second-week schedule.
    if (entry.week === 1) {
        date.add(7, 'day');
    }

    // date.day() can move the date backwards. If it did move the date to August - fix it back to September.
    if (date.date() > 14 && !isSecondSemester) {
        date.add(14, 'day');
    }

    const daystr = date.format('DD');

    const lessonName = entry.names.join(' | ');
    const lecturerName = entry.lecturers.join(' | ');
    const location = entry.locations.join(' | ');

    const formatTime = (time: string) => moment(time, 'HH:mm').utc().format('HH:mm');

    const timeStart = formatTime(lecture_start_time(entry.index));
    const timeEnd = formatTime(lecture_end_time(entry.index));

    return {
        summary: lessonName,
        description: `${lessonName}\nВикладач: ${lecturerName}`,
        start: {
            dateTime: `${moment().year()}${isSecondSemester ? '-02-' : '-09-'}${daystr}T${timeStart}:00.000Z`,
            timeZone: 'Europe/Kiev'
        },
        end: {
            dateTime: `${moment().year()}${isSecondSemester ? '-02-' : '-09-'}${daystr}T${timeEnd}:00.000Z`,
            timeZone: 'Europe/Kiev'
        },
        recurrence: [
            `RRULE:FREQ=WEEKLY;INTERVAL=2;UNTIL=${moment().year()}${isSecondSemester ? '0610' : '1231'}T235959Z`
        ],
        location: `НТУУ "КПІ" (${location})`
    };
};

const lecture_start_time = (index: number): string => {
    switch (index) {
        case 0:
            return '08:30';
        case 1:
            return '10:25';
        case 2:
            return '12:20';
        case 3:
            return '14:15';
        case 4:
            return '16:10';
        case 5:
            return '18:30';
        default:
            return '20:00'; // haha
    }
};

const lecture_end_time = (index: number): string => {
    switch (index) {
        case 0:
            return '10:10';
        case 1:
            return '12:05';
        case 2:
            return '14:00';
        case 3:
            return '15:55';
        case 4:
            return '18:15';
        case 5:
            return '20:00';
        default:
            return '21:30'; // haha
    }
};

const create_calendar = async (access_token: string, summary: string): Promise<CreateCalendarResponse> => {
    return await fetch(`https://www.googleapis.com/calendar/v3/calendars/?access_token=${access_token}`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            summary,
            location: 'NTUU KPI, Kyiv, Ukraine'
        })
    })
        .then(res => res.json())
        .then(res => res as CreateCalendarResponse);
};

const get_google_token = (): Promise<GoogleOAuthToken> => new Promise((resolve, reject) => {    
    const scope = 'https://www.googleapis.com/auth/calendar';
    const redirect_uri = document.location.protocol + '//' + document.location.host + '/oauth';

    const url = `https://accounts.google.com/o/oauth2/v2/auth` +
        `?scope=${scope}` +
        `&response_type=token` +
        `&client_id=${oauthClientId}` +
        `&redirect_uri=${redirect_uri}`;

    const tab = window.open(
        url,
        'Authentication',
        'height=1000,width=1000,modal=yes,alwaysRaised=yes'
    );

    if (tab !== null) {
        const timer = setInterval(() => {
            try {
                const match = /^#access_token=(.*)&token_type/.exec(tab.document.location.hash);
                if (match) {
                    tab.close();
                    clearInterval(timer);
                    resolve(match[1]);
                } else if (tab.document.location.hash === '#error=access_denied') {
                    tab.close();
                    console.log('resolving with error');
                    reject('You have to allow access to your Google profile to use this app.');
                }
            } catch(e) {
                console.log('got error while checking auth tag', e);
            }
        }, 100);
    } else {
        console.error('failed to open oauth tab');
    }
});

const scheduleForGroup = async (groupName: string, studentName: string): Promise<GetScheduleResponse> => new Promise<GetScheduleResponse>((resolve, reject) => {
    const reqUrl = (studentName || '').trim() !== ''
        ? `/schedule/${groupName}?lastName=${studentName}`
        : `/groups/${groupName}`;

    const req = new XMLHttpRequest();
    req.open('GET', reqUrl, true);
    req.onreadystatechange = () => {
        if (req.readyState === 4 && req.status === 200) {
            resolve(JSON.parse(req.responseText) as GetScheduleResponse);
        }
    };
    req.send(null);
});