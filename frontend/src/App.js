import React from 'react';
import {Helmet} from 'react-helmet';

// import Fetchers from './utilities/fetchers.js';
import Router from './components/router.js';
import OuterHeader from './components/outer-header.js';

const App = () =>
      <div className="ma3 mw7">
        <Helmet>
          <meta charSet="utf-8" />
          <title>MMW</title>
        </Helmet>
        <OuterHeader/>
        <Router/>
      </div>; 


export default App;







