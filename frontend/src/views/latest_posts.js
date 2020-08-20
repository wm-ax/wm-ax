import React, { Component, useEffect, useState } from 'react';
import axios from 'axios';
import { Link } from 'react-router-dom';


import { OuterHeader } from '../components/outer-header.js';
import { FRONTEND_URLS, API_URLS } from '../urls.js';


// async function getPosts(setPosts) {
//     try {
//         const response = await axios.get(api_url);
//         setPosts(response.data);
//     } catch(error) {
//         console.log("error", error);
//     }
// }

function LatestPosts(props) {

    const api_url = API_URLS.articles_list;            

    const [posts, setPosts] = useState([]);

    useEffect(() => {
        async function fetchData(url) {
            try {
                const response = await axios.get(url);
                setPosts(await response.data);
            } catch(error) {
                console.log("error", error);
            }
        }
        fetchData(api_url);
    },
              [api_url]);


    return posts === [] ? "no posts found" : (
        // <OuterHeader/>
        <ol>
          {posts.map(
              (post) =>
                  <div>
                    <li>
                      <h3>
                        <Link to={FRONTEND_URLS.article_url(post)}>
                          {post.title}
                        </Link>
                      </h3>
                    </li>
                  </div>
          )}
        </ol>
    );
}




export default LatestPosts;
