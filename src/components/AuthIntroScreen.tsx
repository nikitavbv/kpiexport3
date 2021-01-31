import React from 'react';

type AuthIntroScreenProps = {
    onDone: () => void,
};

export const AuthIntroScreen = (props: AuthIntroScreenProps) => (
    <div>
        <h2>Please grant access</h2>
        <div>
            <img src={'/oauth.png'} alt={'oauth instructions'} />
            <a className='button' onClick={() => props.onDone()}>Ok, continue</a>
        </div>
    </div>
);