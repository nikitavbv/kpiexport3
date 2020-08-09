import React, { useState, Dispatch } from 'react';
import { connect } from 'react-redux';
import { 
    ErrorMessage, ExportInputScreen, ExportInProgressScreen, ExportFinishedScreen 
} from './components';

import './app.css';

// routing and urls do not make sense here.
type Screen = 'input' | 'in_progress' | 'finished';

const mapStateToProps = (state: {}) => ({});

const mapDispatchToProps = (dispatch: Dispatch<{}>) => ({
});

const AppComponent = () => {
    const [error, setError] = useState<string|undefined>(undefined);
    const [screen, setScreen] = useState<Screen>('input');

    return (
        <>
            <main>
                <h1>KPI Exporter</h1>

                { error !== undefined ? <ErrorMessage errorText={error} /> : undefined }
                { screenElementByType(screen) }
            </main>
            <footer>
                by <a href="https://nikitavbv.com">nikitavbv</a>, see <a href="https://github.com/nikitavbv/kpiexport3">Github</a> for source code
            </footer>
        </>
    );
};

const screenElementByType = (type: Screen) => {
    switch (type)
    {
        case 'input':
            return (<ExportInputScreen />);
        case 'in_progress':
            return (<ExportInProgressScreen />);
        case 'finished':
            return (<ExportFinishedScreen />);
    }
};

export const App = connect(mapStateToProps, mapDispatchToProps)(AppComponent);
