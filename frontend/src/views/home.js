import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
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
                  <li key={post.title}>
                      <h3>
                        <Link to={FRONTEND_URLS.article_detail(post.slug)}>
                          {post.title}
                        </Link>
                      </h3>
                    </li>
          )}
        </ol>
    );
}




export default Home;
