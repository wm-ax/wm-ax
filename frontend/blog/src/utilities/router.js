import React from 'react';
import {
    BrowserRouter,
    Switch,
    Route,
    // Link
} from "react-router-dom";

import LatestPosts from '../views/latest_posts.js';
import NewPost from '../views/new_post.js';
import PostDetail from '../views/post_detail.js';

const Router = ({className}) =>
      <div>
        <BrowserRouter>
          <Switch>
            <Route exact path="/">
              <LatestPosts/>
            </Route>
            <Route path="/articles/new">
              <NewPost/>
            </Route>
            <Route>
              <PostDetail  slug="a-first-post"/>
            </Route>
          </Switch>
        </BrowserRouter>
      </div>;


export default Router;



