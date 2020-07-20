import React from 'react';

export const ExportInputScreen = () => (
    <div>
        <InputBlock>
            <label htmlFor="group_name">Group Name:</label>
            <input name="group_name" placeholder="For example: ІП-82" list="groups" />

            <datalist id="groups">
            </datalist>
        </InputBlock>
        <InputBlock>
            <label htmlFor="group_name">Calendar Name:</label>
            <input name="group_name" value="KPI Schedule" />        
        </InputBlock>
        <InputBlock>
            <button id="export_btn">Export to Google Calendar</button>
        </InputBlock>
    </div>
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
        { props.children }
    </div>
);