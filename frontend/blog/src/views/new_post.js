import React, { Component } from 'react';
import {Helmet} from 'react-helmet';

// import striptags from 'striptags';

// import Fetchers from '../utilities/fetchers.js';

// import {ExtendingList, ProfileMini} from '../../components/extending-list.js';
// import {RecommendationsSidebar, RecommendationsSidebarPanel, CommunityMini, TopicMini} from '../../components/recommendations-sidebar.js';

// import {TruncateHtml} from '../../utilities/formatting.js';

// import {settings, api} from '../settings.js';

const API_URL = `http://localhost:8000/api/article`;

const ARTICLE = {'slug': 'test-slug', 'content': 'this is an article about crap'};

class NewPost extends Component {

     constructor(props) {
        super(props);
        this.state = {};
    }

    submitPost() {
        const url = `${API_URL}`;
        const request_info = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(ARTICLE),
        };
        fetch(url, request_info)
            .then(response => response.json())
            .then(data => {
                console.log('Success:', data);
            })
            .catch((error) => {
                console.error('Error:', error);
            });
    }
    
    getPosts() {
        const url = 'http://localhost:8000/api/article';
        fetch(url)
            .then(response => {
                if (response.ok) {
                    return response.json();
                } else {
                    throw new Error("WTF");
                }
            })
            .then(data => this.setState({latest_posts: data}))
            .catch(error => this.setState({ error }));
    }


    componentDidMount() {
        this.submitPost();
        // this.getPosts();
    }

    render() {
        // this.submitPost();
        const {latest_posts
              }
              = this.state;
        if (!latest_posts) {
            return "no latest posts";
            // return JSON.stringify(this.state);
        }
        return (
            <div className="page">
              <Helmet>
                <meta charSet="utf-8" />
                <title>MMW - Latest Posts</title>
              </Helmet>
              <hr/>
              <div className="cf fl">
                <div className="fl w-90 pr2 justify-center">
                  <div className="latest_posts">
                    <h2>latest posts, aloha</h2>
                    <ol className="list pl0 mt2">
                      {latest_posts.map(
                          (post) =>
                              <div className="post_mini">
                                <li>
                                  <h3>
                                    {post.title}
                                  </h3>
                                  <p>
                                    {post.content}
                                  </p>
                                </li>
                              </div>
                      )}
                    </ol>
                  </div>;

                </div>
              </div>
            </div>
        );
    }
}



export default NewPost;
