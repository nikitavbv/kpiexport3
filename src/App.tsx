import React, { useState } from 'react';
import { 
    ErrorMessage, ExportInputScreen, ExportInProgressScreen, ExportFinishedScreen 
} from './components';

import './app.css';
import { GetGroupsResponse, GetScheduleResponse, GoogleOAuthToken } from './types';
import { oauthClientId } from './constants';

// routing and urls do not make sense here.
type Screen = 'input' | 'in_progress' | 'finished';

type ExportFunction = (groupName: string, calendarName: string) => void;

export const App = () => {
    const [error, setError] = useState<string|undefined>(undefined);
    const [screen, setScreen] = useState<Screen>('input');

    const exportFunction = exportSchedule;

    return (
        <>
            <main>
                <h1>KPI Exporter</h1>

                { error !== undefined ? <ErrorMessage errorText={error} /> : undefined }
                { screenElementByType(screen, exportFunction) }
            </main>
            <footer>
                by <a href="https://nikitavbv.com">nikitavbv</a>, see <a href="https://github.com/nikitavbv/kpiexport3">Github</a> for source code
            </footer>
        </>
    );
};

const screenElementByType = (type: Screen, onExportStart: ExportFunction) => {
    switch (type)
    {
        case 'input':
            return (<ExportInputScreen onSubmit={onExportStart} />);
        case 'in_progress':
            return (<ExportInProgressScreen />);
        case 'finished':
            return (<ExportFinishedScreen />);
    }
};

const exportSchedule = async (groupName: string, calendarName: string) => {
    let token = await get_google_token();
    let schedule = await scheduleForGroup(groupName);
    // TODO: add export
};

const get_google_token = (): Promise<GoogleOAuthToken> => new Promise((resolve, reject) => {    
    const scope = 'https://www.googleapis.com/auth/calendar';
    const redirect_uri = document.location.protocol + '//' + document.location.host + '/oauth';

    console.log(redirect_uri);

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
            const match = /^#access_token=(.*)&token_type/.exec(tab.document.location.hash);
            if (match) {
                tab.close();
                clearInterval(timer);
                resolve(match[1]);
            } else if (tab.document.location.hash === '#error=access_denied') {
                tab.close();
                reject('You have to allow access to your Google profile to use this app.');
            }
        }, 100);
    } else {
        console.error('failed to open oauth tab');
    }
});

const scheduleForGroup = async (groupName: string) => new Promise((resolve, reject) => {
    const req = new XMLHttpRequest();
    req.open('GET', `/groups/${groupName}`, true);
    req.onreadystatechange = () => {
        if (req.readyState === 4 && req.status === 200) {
            resolve(JSON.parse(req.responseText) as GetScheduleResponse);
        }
    };
    req.send(null);
});