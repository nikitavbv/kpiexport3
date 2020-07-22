// import { ACTION1, ACTION2 } from './actions';

export type GroupAction = SetGroupAction | SetPossibleGroupsAction;

export type SetGroupAction = {
    type: 'set_group',
    group: string,
};

export type SetPossibleGroupsAction = {
    type: 'set_possible_groups',
    groups: string[],
};

export type GroupState = {
    group: string,
    groups: string[],
};

const defaultState: GroupState = {
    group: '',
    groups: [],
};

export default (state = defaultState, action: GroupAction): GroupState => {
    switch (action.type) {
        default:
            return state;
    }
};