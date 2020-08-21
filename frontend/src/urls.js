import { FRONTEND_ROOT_URL } from './settings';
import { API_ROOT_URL } from './settings';


const article_detail = (slug) => {
    return FRONTEND_ROOT_URL + '/articles/' + slug;
}
const compose = FRONTEND_ROOT_URL + '/compose';
const home = FRONTEND_ROOT_URL;

export const FRONTEND_URLS = {article_detail, compose, home};


export const API_URLS = {article_list : API_ROOT_URL + `/article`,
                         article_compose : API_ROOT_URL + `/article/`,
                         article_detail : slug => API_ROOT_URL + `/article/` + slug,
                        };


// const make_url_getter = (url_data) =>
//       (urlName, ...params) =>
//       frontend_url_data[urlName](params);

// export const frontend_urls = make_url_getter(frontend_url_data);
// export const backend_urls = make_url_getter(backend_url_data);

