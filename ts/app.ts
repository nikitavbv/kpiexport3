const client_id = '1021403911825-9ef33mqqvhhpt39dke73ms2j4oa2jrkm.apps.googleusercontent.com';

type GroupOkr = 'bachelor';

type GroupInfo = {
    group_id: number,
    group_full_name: string,
    group_prefix: string,
    group_okr: GroupOkr,
    group_type: 'daily' | 'extramural',
    group_url: string,
};

type GetGroupsResponse = {
    statusCode: number,
    timeStamp: number,
    message: string,
    debugInfo: {},
    meta: {
        total_count: string,
        offset: {},
        limit: {},
    },
    data: GroupInfo[],
};

type GoogleOAuthToken = string;

const get_groups = (): Promise<GroupInfo[]> => new Promise((resolve, reject) => {
    const req = new XMLHttpRequest();
    req.open('GET', '//api.rozklad.org.ua/v2/groups/?filter={"showAll":true}', true);
    req.onreadystatechange = () => {
        if (req.readyState === 4 && req.status === 200) {
            const parsed = JSON.parse(req.responseText) as GetGroupsResponse;
            resolve(parsed.data);
        }
    };
    req.send(null);
});

const group_to_datalist_option = (group: GroupInfo): HTMLOptionElement => {
    const element = document.createElement('option');
    element.value = group.group_full_name;
    return element;
};

const deduplicate_groups = (groups: GroupInfo[]): GroupInfo[] => {
    const result = [];
    const groups_by_id = {};
    const ids = {};
    groups.forEach(group => {
        const id = group.group_id;
        const full_name = group.group_full_name;

        groups_by_id[id] = group;

        if (full_name in ids) {
            ids[full_name].push(id);
        } else {
            ids[full_name] = [id];
        }
    });

    Object.keys(ids).forEach(groupName => {
        const groupIds = ids[groupName];
        if (groupIds.length == 1) {
            result.push(groups_by_id[groupIds[0]]);
        } else {
            groupIds.map(id => groups_by_id[id]).map(g => ({
                ...g,
                group_full_name: `${g.group_full_name} [${g.group_id}]`,
            })).forEach(g => result.push(g));
        }
    });

    return result;
};

const get_google_token = (): Promise<GoogleOAuthToken> => new Promise((resolve, reject) => {
    const scope = 'https://www.googleapis.com/auth/calendar';
    const redirect_uri = document.location.protocol + '//' + document.location.host + '/oauth';

    const url = `https://accounts.google.com/o/oauth2/v2/auth` +
        `?scope=${scope}` +
        `&response_type=token` +
        `&client_id=${client_id}` +
        `&redirect_uri=${redirect_uri}`;

    const tab = window.open(
        url,
        'Authentication',
        'height=1000,width=1000,modal=yes,alwaysRaised=yes'
    );

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
});

const display_error = (msg: string) => {
    const error_div = document.getElementById('error_msg') as HTMLDivElement;
    error_div.innerHTML = msg;
    error_div.style.display = 'inherit';
};

window.onload = () => {
    let groups = [];

    const groups_datalist = document.getElementById('groups') as HTMLDataListElement;
    const export_btn = document.getElementById('export_btn') as HTMLButtonElement;

    export_btn.onclick = async () => {
        console.log('export clicked');
        try {
            const google_token = await get_google_token();
            console.log('google token is', google_token);
        } catch(e) {
            display_error(`error: ${e}`);
        }
    };

    get_groups().then(g => {
        groups = deduplicate_groups(g);
        groups_datalist.innerHTML = '';
        groups.map(group_to_datalist_option).forEach(g => groups_datalist.appendChild(g));
    });
};
