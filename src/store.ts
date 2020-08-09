import { createStore } from 'redux';
import { rootReducer } from './reducer';

export const store = createStore(rootReducer, JSON.parse(localStorage.state || '{}'));
store.subscribe(() => localStorage.state = JSON.stringify(store.getState()));
