import React, { useReducer } from 'react';
import axios from 'axios';

import { API_URLS } from '../urls';



const initialArticle = {title: "", content: ""};

const [minTitleLength, maxTitleLength] = [1, 100];
const [minArticleLength, maxArticleLength] = [3, 100000];


function reducer (article, action) {
    switch (action.type) {
    case 'inputChange':
        return {...article,
                [action.name]: action.value};
    case 'submit':
        axios.post(API_URLS.article_compose,
                  article);        
        return article;
    default:
        throw new Error(`dispatcher didn't match the given action type ${action.type}`);
    }
}


function ArticleCreateForm() {
    const [, articleDispatch] = useReducer(reducer, initialArticle);
    const onChange = (event) => articleDispatch(
                {type: 'inputChange',
                 name: `${event.target.name}`,
                 value: `${event.target.value}`});

    return (
        <form
          onSubmit={() => articleDispatch({type: 'submit' })}
        >
          <label htmlFor="title">
            title:
          </label>
          <input
            type="text"
            id="title"
            name="title"
            required
            minLength={minTitleLength}
            maxLength={maxTitleLength}
            onChange={onChange}
          />
          <br/>
          <label htmlFor="title">
            content:
          </label>
          <textarea
            id="articleContent"
            name="content"
            required
            minLength={minArticleLength}
            maxLength={maxArticleLength}
            onChange={onChange}
          />
          <br/>
          <button type="submit">
            submit
          </button>
        </form>
    );
}

export default ArticleCreateForm;
