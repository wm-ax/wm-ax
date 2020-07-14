import React, { Component, useEffect, useState } from 'react';
import { useParams } from 'react-router';
import axios from 'axios';

import { API_URL } from '../settings.js';

function PostDetail(props) {

    const [post, setPost] = useState(null);

    let slug = props.slug;

    const endpoint_url = API_URL + `/article/${slug}`;

    useEffect(() => {
        async function getPost(url) {
            try {
                const response = await axios.get(url);
                setPost(await response.data);
            } catch(error) {
                console.log("error", error);
            }
        }
        getPost(endpoint_url);
    }, 
              [endpoint_url]);


    return post === null ? "no post here" : (
        <div>
          <h3>{post.title}</h3>
          <p>{post.content}</p>
        </div>
    );
}




export default PostDetail;
