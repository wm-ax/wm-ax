import axios from 'axios';

// const LOCAL_ROCKET = {
//     API_URL : 'http://localhost:8000/api',
//     axios_with_credentials : true,
//     AXIOS : axios,
// }

export const API_ROOT_URL = process.env.WM_API_URL || `http://localhost:8000/api`;
export const FRONTEND_ROOT_URL = process.env.WM_FRONTEND_URL || `http://localhost:3000`;

// export const settings = LOCAL_ROCKET;

// axios.withCredentials = settings['axios_with_credentials'];

export const api = axios;

// export default settings;
