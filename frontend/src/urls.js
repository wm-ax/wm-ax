// import { FRONTEND_ROOT_URL } from './settings';
import { API_ROOT_URL } from './settings';


// export const FRONTEND_URLS = {home: FRONTEND_ROOT_URL,
//                               compose: FRONTEND_ROOT_URL + '/compose',
//                               article_detail : (slug) => 
//                                   FRONTEND_ROOT_URL + '/articles/' + slug,
//                               };

export const FRONTEND_URLS = {home: '/',
                              compose: '/compose',
                              article_detail : (slug) => 
                                  '/articles/' + slug,
                              // article_detail : (slug) => '/articles/:slug',
                             };



export const API_URLS = {article_list : API_ROOT_URL + `/article`,
                         article_compose : API_ROOT_URL + `/article/`,
                         article_detail : slug => API_ROOT_URL + `/article/` + slug,
                        };


// const make_url_getter = (url_data) =>
//       (urlName, ...params) =>
//       url_data[urlName](params);

// export const frontend_urls = make_url_getter(frontend_url_data);
// export const backend_urls = make_url_getter(backend_url_data);

