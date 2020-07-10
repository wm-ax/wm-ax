import React, { Component, useEffect, useState } from 'react';
import axios from 'axios';


const API_URL = `http://localhost:8000/api`;

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


    return posts == [] ? "no posts found" : (
        <ol>
          {posts.map(
              (post) =>
                  <div>
                    <li>
                      <h3>
                        {post.title}
                      </h3>
                      <p>
                        {post.content}
                      </p>
                    </li>
                  </div>
          )}
        </ol>
    );
}




export default LatestPosts;
