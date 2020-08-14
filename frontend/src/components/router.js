import React from 'react';
import {
    BrowserRouter,
    Switch,
    Route,
    // Link
} from "react-router-dom";

import Home from '../views/home.js';
import NewPost from '../views/new_post.js';
import PostDetail from '../views/post_detail.js';

const Router = ({className}) =>
      <div>
        <BrowserRouter>
          <Switch>
            <Route exact path="/">
              <Home/>
            </Route>
            <Route exact path="/articles/new">
              <NewPost/>
            </Route>
            <Route exact path="/articles/:slug" component={PostDetail}>
            </Route>
          </Switch>
        </BrowserRouter>
      </div>;


export default Router;



