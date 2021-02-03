import React, {useEffect, useState} from 'react';
import { GetGroupsResponse } from '../types';

export type ExportInputScreenProps = {
    onSubmit: (groupName: string, calendarName: string, studentName: string) => void,
};

export const ExportInputScreen = (props: ExportInputScreenProps) => {
    const [groups, setGroups] = useState<string[]>([]);

    const [selectedGroup, updateSelectedGroup] = useState<string>(localStorage.group || '');
    const [calendarName, updateCalendarName] = useState<string>(localStorage.calendar || 'KPI Schedule');
    const [studentName, updateStudentName] = useState<string>(localStorage.studentName || '');

    useEffect(() => {
        getGroups().then(setGroups);
    }, []);

    const autoCompleteStyle: React.CSSProperties = {
        display: 'inline-block',

        backgroundColor: '#f1f2f6',
        color: '#202125',

        borderRadius: '5px',
        padding: '8px',
        marginRight: '8px',

        fontStyle: 'italic',
        userSelect: 'none',
        cursor: 'pointer',
    };

    const autoCompletions = groups.filter(t => {
        const groupLower = t.toLowerCase();
        const selectedLower = selectedGroup.toLowerCase();

        return t !== selectedGroup && (groupLower.startsWith(selectedLower) || transliterate(groupLower).startsWith(transliterate(selectedLower)))
    }).slice(0, 5);

    const completionElement = autoCompletions.length > 0 ? (
        <div style={{marginTop: '20px', maxWidth: '340px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap'}}>
            { autoCompletions.map(v => <span style={autoCompleteStyle} onClick={() => {
                localStorage.group = v;
                updateSelectedGroup(v);
            }}>{ v }</span>)}
        </div>
    ) : (<></>);

    const isCustomScheduleSupported = selectedGroup === 'ІП-82';
    const studentNameInput = (
        <InputBlock>
            <label htmlFor="student_name">Your last name (optional):</label>
            <input name="student_name" placeholder="For example: Волобуев" value={studentName} onChange={e => {
                const studentName = e.target.value;
                localStorage.studentName = studentName;
                updateStudentName(studentName);
            }}/>
        </InputBlock>
    );

    return (
        <div>
            <InputBlock>
                <label htmlFor="group_name">Group Name:</label>
                <input name="group_name" placeholder="For example: ІП-82" list="groups" value={selectedGroup} onChange={e => {
                    const group = e.target.value;
                    localStorage.group = group;
                    updateSelectedGroup(group);
                }}/>

                { completionElement }
            </InputBlock>
            { isCustomScheduleSupported ? studentNameInput : undefined }
            <InputBlock>
                <label htmlFor="group_name">Calendar Name:</label>
                <input name="group_name" value={calendarName} onChange={e => {
                    const calendarName = e.target.value;
                    localStorage.calendar = calendarName;
                    updateCalendarName(calendarName);
                }} />
            </InputBlock>
            <InputBlock>
                <button
                    disabled={groups.indexOf(selectedGroup) === -1}
                    onClick={() => props.onSubmit(selectedGroup, calendarName, studentName)}>
                    Export to Google Calendar
                </button>
            </InputBlock>
        </div>
    );
};

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

const lettersTransliteration = (() => {
    let a: Record<string, string> = {};

    a["Ё"]="YO";a["Й"]="I";a["Ц"]="TS";a["У"]="U";a["К"]="K";a["Е"]="E";a["Н"]="N";a["Г"]="G";a["Ш"]="SH";a["Щ"]="SCH";a["З"]="Z";a["Х"]="H";a["Ъ"]="'";
    a["ё"]="yo";a["й"]="i";a["ц"]="ts";a["у"]="u";a["к"]="k";a["е"]="e";a["н"]="n";a["г"]="g";a["ш"]="sh";a["щ"]="sch";a["з"]="z";a["х"]="h";a["ъ"]="'";
    a["Ф"]="F";a["Ы"]="I";a["В"]="V";a["А"]="a";a["П"]="P";a["Р"]="R";a["О"]="O";a["Л"]="L";a["Д"]="D";a["Ж"]="ZH";a["Э"]="E";
    a["ф"]="f";a["ы"]="i";a["в"]="v";a["а"]="a";a["п"]="p";a["р"]="r";a["о"]="o";a["л"]="l";a["д"]="d";a["ж"]="zh";a["э"]="e";
    a["Я"]="Ya";a["Ч"]="CH";a["С"]="S";a["М"]="M";a["И"]="I";a["Т"]="T";a["Ь"]="'";a["Б"]="B";a["Ю"]="YU";
    a["я"]="ya";a["ч"]="ch";a["с"]="s";a["м"]="m";a["и"]="i";a["т"]="t";a["ь"]="'";a["б"]="b";a["ю"]="yu";
    a["І"]="I";a["і"]='i';

    return a;
})();

const transliterate = (word: string) => {
    let answer = '';

    for (let i of word){
        if (lettersTransliteration[i] === undefined){
            answer += i;
        } else {
            answer += lettersTransliteration[i];
        }
    }

    return answer;
}
