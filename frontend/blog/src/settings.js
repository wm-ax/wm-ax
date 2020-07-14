import axios from 'axios';

const LOCAL_ROCKET = {
    API_URL : 'http://localhost:8000/api',
    axios_with_credentials : true,
    AXIOS : axios,
}

export const API_URL = `http://localhost:8000/api`;

// const FOR_PRISM = {
//     API_URL : 'http://127.0.0.1:4010/api',
//     axios_with_credentials : false,
// }

export const settings = LOCAL_ROCKET;
// export const settings = FOR_PRISM;

axios.withCredentials = settings['axios_with_credentials'];

export const api = axios;

export default settings;
