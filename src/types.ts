// routing and urls do not make sense here.
export type Screen = 'input' | 'in_progress' | 'finished' | 'auth_intro';

export type GoogleOAuthToken = string;

export type GetGroupsResponse = string[];
export type GetScheduleResponse = {
    entries: GroupScheduleEntry[],
};
export type GroupScheduleEntry = {
    week: number,
    day: number,
    index: number,
    names: string[],
    lecturers: string[],
    locations: string[],
};

export type CreateCalendarResponse = {
    id: string
};

export type CalendarEntry = {
    summary: string,
    description: string,
    start: {
        dateTime: string,
        timeZone: string,
    },
    end: {
        dateTime: string,
        timeZone: string,
    },
    recurrence: string[],
    location: string,
};

export type ExportFunction = (groupName: string, calendarName: string) => void;
