import React, { useEffect, useState } from 'react';
import axios from 'axios';

import { API_URLS, FRONTEND_URLS } from '../urls.js';


function Home(props) {

    const api_url = API_URLS.article_list;

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
        <ol>
          {posts.map(
              (post) =>
                  <div>
                  <li key={post.title}>
                      <h3>
                        <a href={FRONTEND_URLS.article_detail(post.slug)}>
                          {post.title}
                        </a>
                      </h3>
                    </li>
                  </div>
          )}
        </ol>
    );
}




export default Home;
