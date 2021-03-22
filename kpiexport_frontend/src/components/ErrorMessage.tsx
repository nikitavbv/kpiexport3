import React from 'react';

type ErrorMessageProps = {
    errorText: string,
};

export const ErrorMessage = (props: ErrorMessageProps) => (
    <div style={{
        width: '340px',
        margin: '0 auto',
        backgroundColor: '#e74c3c',
        textAlign: 'center',
        padding: '8px',
        borderRadius: '5px',
    }}>
        { props.errorText }    
    </div>
);
