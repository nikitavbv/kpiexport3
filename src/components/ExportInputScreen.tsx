import React, { useState } from 'react';
import { GetGroupsResponse } from '../types';

export type ExportInputScreenProps = {
    onSubmit: (groupName: string, calendarName: string) => void,
};

export const ExportInputScreen = (props: ExportInputScreenProps) => {
    const [groups, setGroups] = useState<string[]>([]);
    const [loadingGroups, setLoadingGroups] = useState<boolean>(false);

    const [selectedGroup, updateSelectedGroup] = useState<string>(localStorage.group || '');
    const [calendarName, updateCalendarName] = useState<string>(localStorage.calendar || 'KPI Schedule');

    if (!loadingGroups) {
        getGroups().then(setGroups);
        setLoadingGroups(true);
    }

    return (
        <div>
            <InputBlock>
                <label htmlFor="group_name">Group Name:</label>
                <input name="group_name" placeholder="For example: ІП-82" list="groups" value={selectedGroup} onChange={e => {
                    const group = e.target.value;
                    localStorage.group = group;
                    updateSelectedGroup(group);
                }} />

                <datalist id="groups">
                    { groups.map(GroupDataListOption) }
                </datalist>
            </InputBlock>
            <InputBlock>
                <label htmlFor="group_name">Calendar Name:</label>
                <input name="group_name" value={calendarName} onChange={e => {
                    const calendarName = e.target.value;
                    localStorage.calendar = calendarName;
                    updateCalendarName(calendarName);
                }} />
            </InputBlock>
            <InputBlock>
                <button onClick={() => props.onSubmit(selectedGroup, calendarName)} >Export to Google Calendar</button>
            </InputBlock>
        </div>
    );
};

const GroupDataListOption = (group: string) => (
    <option key={group} value={group} />
);

type InputBlockProps = {
    children: JSX.Element | JSX.Element[],
};

const InputBlock = (props: InputBlockProps) => (
    <div style={{
        display: 'block',
        margin: '0 auto',
        padding: '8px',
        width: 'fit-content',
    }}>
        {props.children}
    </div>
);

const getGroups = (): Promise<string[]> => new Promise((resolve, reject) => {
    const req = new XMLHttpRequest();
    req.open('GET', '/groups', true);
    req.onreadystatechange = () => {
        if (req.readyState === 4 && req.status === 200) {
            resolve(JSON.parse(req.responseText) as GetGroupsResponse);
        }
    };
    req.send(null);
});
