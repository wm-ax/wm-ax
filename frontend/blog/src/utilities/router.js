import React from 'react';
import {
    BrowserRouter,
    Switch,
    Route,
    // Link
} from "react-router-dom";

import LatestPosts from '../views/latest_posts.js';

const Router = ({className}) =>
      <div>
      <BrowserRouter>
      <Switch>
        <Route path=""
               render={(props)=>
                       <LatestPosts
                         {...props}
                       />}
        />
      </Switch>
      </BrowserRouter>
    </div>;


export default Router;



