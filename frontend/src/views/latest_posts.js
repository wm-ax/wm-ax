import React, { Component, useEffect, useState } from 'react';
import axios from 'axios';
import { Link } from 'react-router-dom';

import { API_URL } from '../settings.js';

const ENDPOINT_URL = `/article`;


async function getPosts(setPosts) {
    const url = API_URL+ENDPOINT_URL;
    try {
        const response = await axios.get(url);
        setPosts(response.data);
    } catch(error) {
        console.log("error", error);
    }
}



function LatestPosts(props) {

    const URL = API_URL+ENDPOINT_URL;            

    const [posts, setPosts] = useState([]);

    useEffect(() => {
        async function fetchData(url) {
            try {
                const response = await axios.get(URL);
                setPosts(await response.data);
            } catch(error) {
                console.log("error", error);
            }
        }
        fetchData(URL);
    },
              [URL]);


    return posts === [] ? "no posts found" : (
        <ol>
          {posts.map(
              (post) =>
                  <div>
                    <li>
                      <h3>
                        <Link to={'articles/'+post.slug}>
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
