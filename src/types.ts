// routing and urls do not make sense here.
export type Screen = 'input' | 'in_progress' | 'finished';

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