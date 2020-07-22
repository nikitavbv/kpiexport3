import { combineReducers } from 'redux';

import group from './reducers/group';

export const rootReducer = combineReducers({
    group,
});