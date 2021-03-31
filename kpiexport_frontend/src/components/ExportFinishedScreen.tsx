import React from 'react';

import {Screen} from '../types';

type ExportFinishedScreenProps = {
    setScreen: (s: Screen) => void,
};

export const ExportFinishedScreen = (props: ExportFinishedScreenProps) => (
    <div>
        <h2>Export finished!</h2>
        <div>
            <a className='button' href='https://calendar.google.com'>Go to your calendar</a>
            <button className='button' onClick={() => props.setScreen('input')}>Back</button>
        </div>
    </div>
);
