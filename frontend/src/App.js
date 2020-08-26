import React from 'react';
import { BrowserRouter as Router,
         Switch,
         Route, } from 'react-router-dom';
import { Helmet } from 'react-helmet';

import { FRONTEND_URLS } from './urls.js';
import Home  from './views/home.js';
import  NewPost  from './views/new_post.js';
import  PostDetail  from './views/post_detail.js';
import OuterHeader from './components/outer-header.js';

const App = () =>
      <Router>
        <div className="ma3 mw7">
          <Helmet>
            <meta charSet="utf-8" />
            <title>MMW</title>
          </Helmet>
          <OuterHeader/>
        </div>
        <div>
          <Switch>
            <Route exact path={FRONTEND_URLS.home}>
              <Home/>
            </Route>
            <Route exact path={FRONTEND_URLS.compose}>
              <NewPost/>
            </Route>
            <Route exact path={FRONTEND_URLS.article_detail(':slug')}
                   component={PostDetail}>
            </Route>
          </Switch>
        </div>
      </Router>;


// const App = () =>
//       <Router>
//         <div className="ma3 mw7">
//           <Helmet>
//             <meta charSet="utf-8" />
//             <title>MMW</title>
//           </Helmet>
//           <OuterHeader/>
//         </div>
//         <div>
//           <Switch>
//             <Route exact path="/">
//               <Home/>
//             </Route>
//             <Route exact path="/compose">
//               <NewPost/>
//             </Route>
//             <Route exact path="/articles/:slug" component={PostDetail}>
//             </Route>
//           </Switch>
//         </div>
//       </Router>;

export default App;








