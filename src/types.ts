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