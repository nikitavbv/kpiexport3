import React from 'react';
import { 
    ErrorMessage, ExportInputScreen, ExportInProgressScreen, ExportFinishedScreen 
} from './components';

import './app.css';

const App = () => {
    return (
        <>
            <main>
                <h1>KPI Exporter</h1>

                <ErrorMessage errorText={'some error text'} />

                <ExportInputScreen />
                <ExportInProgressScreen />
                <ExportFinishedScreen />
            </main>
            <footer>
                by <a href="https://nikitavbv.com">nikitavbv</a>, see <a href="https://github.com/nikitavbv/kpiexport3">Github</a> for source code
            </footer>
        </>
    );
};

export default App;
