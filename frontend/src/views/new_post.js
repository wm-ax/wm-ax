import React from 'react';
import {Helmet} from 'react-helmet';
// import LatestPosts from './latest_posts.js';
import ArticleCreateForm from '../components/article-create-form.js';


const NewPost = () =>
      <div className="page">
        <Helmet>
          <meta charSet="utf-8" />
          <title>mmw - compose</title>
        </Helmet>
        <hr/>
        <div className="cf fl">
          <div className="fl w-90 pr2 justify-center">
            <h2>compose</h2>
            <ArticleCreateForm/>
          </div>
        </div>
      </div>;


export default NewPost;
