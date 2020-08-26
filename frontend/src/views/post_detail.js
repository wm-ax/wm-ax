import React, { useEffect, useState } from 'react';
import axios from 'axios';
import ReactMarkdown from 'react-markdown';

import { API_URLS } from '../urls.js';
import CodeBlock from '../utilities/codeblock';

// const ReactMarkdown = require('react-markdown')


function PostDetail(props) {

    const [post, setPost] = useState(null);

    let slug = props.match.params.slug;

    const api_url = API_URLS.article_detail(slug);

    useEffect(() => {
        async function getPost(url) {
            try {
                const response = await axios.get(url);
                setPost(await response.data);
            } catch(error) {
                console.log("error", error);
            }
        }
        getPost(api_url);
    }, 
              [api_url]);


    return post === null ? "no post here" : (
        <div>
          <h3>{post.title}</h3>
          <ReactMarkdown
            source={post.content}
            renderers={{code: CodeBlock}}
          />
          {/*   <p>{post.content}</p> */}
          {/* <h3>{post.title}</h3> */}
          {/* <p>{post.content}</p> */}
        </div>
    );
}



export default PostDetail;
