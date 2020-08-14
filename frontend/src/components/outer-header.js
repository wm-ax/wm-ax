import React from 'react';
import Search from '../components/search.js';

const OuterHeader = () =>
      <div className="flex justify-between">
        <div className="flex">
          <nav className="views-nav">
            <a href="/" className="link dib black dim mr2 mr3-ns">
              home
            </a> 
            <a href="/recipes" className="link dib black dim mr2 mr3-ns">
              recipes
            </a>
            <a href="/pictures" className="link dib black dim mr2 mr3-ns">
              pictures
            </a>
            <a href="/articles/new" className="link dib black dim mr2 mr3-ns">
              compose
            </a>
          </nav>
          <Search className="mr3"/>
        </div>
        <nav className="auth-nav">
          <div className="flex">
            <a className="link dib black dim mr3 mr4-ns" href="#0">Sign In</a>
            <a className="link dib black dim mr3 mr4-ns" href="#0">Sign Up</a>
          </div>
        </nav>
      </div>
      ;

export default OuterHeader;
