import React from 'react';

type ExportInProgressScreenProps = {
    progressCurrent: number,
    progressTotal: number,
};

export const ExportInProgressScreen = (props: ExportInProgressScreenProps) => {
    const totalWidth = 500;
    const width = totalWidth / props.progressTotal * props.progressCurrent;

    return (
        <div>
            <h2>Export in progress...</h2>
            <div id="progress_border" style={{width: `${totalWidth}px`}}>
                <div id="progress" style={{width: `${width}px`}}></div>
            </div>
        </div>
    );
};