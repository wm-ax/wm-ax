import React from 'react';
import {
    BrowserRouter,
    Switch,
    Route,
    // Link
} from "react-router-dom";

import LatestPosts from '../views/latest_posts.js';
import NewPost from '../views/new_post.js';

const Router = ({className}) =>
      <div>
      <BrowserRouter>
      <Switch>
        <Route path="/articles/new"
               render={(props)=>
                       <NewPost
                         {...props}
                       />}
        />
        <Route exact path="/"
               render={(props)=>
                       <LatestPosts/>}
        />
      </Switch>
      </BrowserRouter>
    </div>;


export default Router;



