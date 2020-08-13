import React from 'react';

import {TruncateHtml, HtmlToReact} from '../utilities/formatting.js';

export const PaginatedLongListBrowser = ({headingStr,
                                          fieldData,
                                          paginator,
                                          itemRenderer
                                         }) =>
    <div className="center mt3">
      <div className="flex justify-between">
        <b className="f3 ml2 mb2">{headingStr}</b>
        <div className="">
          <input
            className="long-list-filter"
            type="text"
            name={fieldData.name}
            label="filter"
            onChange={fieldData.on_change}
          />
          <PageBrowser paginator={paginator}/>
        </div>
      </div>
      <div className="mw9">
        <div className="cf">
          {paginator.contents.map(itemRenderer)}
        </div>
      </div>
    </div>;


const PageBrowser = ({paginator}) =>
      <span className="dib mr0 ml1">
        <PageBackButton
          onClick={paginator.pagers.back}
          hasNext={paginator.current_page>0}
        />
        <PageLocation paginator={paginator}/>
        {/* <PageJumpButton onClick={paginator.pagers.jump} label=/> */}
        <PageForwardButton
          onClick={paginator.pagers.forward}
          hasNext={paginator.current_page + 1 < paginator.num_pages }/>
      </span>;

const PageLocation = ({paginator}) => 
      <span className="page-location">
        <span className="dib">
          {
              paginator.num_pages == 0
                  ? null
                  : (paginator.current_page + 1) + "/" + paginator.num_pages
          }
        </span>
      </span>;


const PageBackButton = ({onClick, hasNext}) => 
    hasNext ?
        <a onClick={onClick} className="f3 link dim ph1 pv2 mb2 dib black">
          &#8592;
        </a>
    :
    <span>
      &emsp;&emsp;
    </span>;


const PageForwardButton = ({onClick, hasNext}) =>
    hasNext ?
        <a onClick={onClick} className="f3 link dim ph1 pv2 mb2 dib black">
          &#8594;
        </a>
    :
    <span>
      &emsp;&emsp;
    </span>;


const PageJumpButton = ({onClick, label}) =>
      <a onClick={onClick} className="f3 dim ba ph3 pv2 mb2 dib black">
        {label}
      </a>;




export const LongListBrowser = ({headingStr,
                                 fieldData,
                                 itemRenderer}) =>
    <div className="center mt3">
      <div className="flex justify-between">
        <b className="f3 ml2 mb2">{headingStr}</b>
        <div className="">
          <input
            type="text"
            name={fieldData.name}
            label="filter"
            onChange={fieldData.on_change}
          />
          <span className="f7 mt1 mr0">
          </span>
        </div>
      </div>
      <div className="mw9">
        <div className="cf">
          {fieldData.contents.map(itemRenderer)}
        </div>
      </div>
    </div>;


export const TopicMini = (topic) =>
    <div className="fl w-100 w-50-ns pa1" key={topic.slug}>
      <div className="bg-white">
        <a href={"/topics/" + topic.slug}
           className="link black dim">
          <b>{topic.name}</b>
          <br/>
          <TruncateHtml html={topic.excerpt} lines="3"/>
        </a>
      </div>
    </div>;


export const CommunityMini = (community) =>
    <div className="fl w-100 w-50-ns pa1" key={community.slug}>
      <div className="bg-white">
        <a href={"/communities/" + community.slug}
           className="link black dim">
          <b>{community.name}</b>
          <br/>
          {/* <HtmlToReact html={community.excerpt}/> */}
        </a>
      </div>
    </div>;


const FieldFilterClass = class {
    // ancestor View component should bind 'this' to itself

    filteredField(name) {

        const filterTerm = this.state.fieldFilters[name];
        const theFilter = (
                (item) =>
                (item.name+item.excerpt)
                .toLowerCase()
                .includes(filterTerm.toLowerCase()));
        const contents = this.state[name].filter(theFilter);

        const on_change = (event) => {
            const oldFilters = this.state.fieldFilters;
            const newFilter = {[event.target.name] : event.target.value };
            const fieldFilters = {...oldFilters, ...newFilter};
            const fieldPageNums = {...this.state['fieldPageNums'],
                                   'all_topics': 0};
            return this.setState((state, props) => ({fieldFilters, fieldPageNums}));
        };

        return {name,
                contents,
                on_change};
    }
};

export const FieldFilter = new FieldFilterClass();

const FieldPaginatorClass = class {
    // bind 'this' to ViewComponent

    paginatedField(name, data_to_paginate) {
        
        const PAGE_SIZE=20;

        const pageNum = this.state.fieldPageNums[name];

        const paginateField = (items, pageSize) => {
            var paginated = [];
            var i = 0, num_pages = 0;
            for (i = 0; i < items.length; i += pageSize) {
                paginated.push(items.slice(i, i + pageSize));
                num_pages += 1;
            }
            return [paginated, num_pages];
        };

        const [contents_paginated, num_pages] =
              paginateField(data_to_paginate, PAGE_SIZE);
        const contents = num_pages > 0 ? contents_paginated[pageNum] : [];
        // recalculate only on: initial page load, or change in fieldFilter

        const set_page_num = (pageNum) =>  {
            const oldPageNums = this.state.fieldPaginators;
            const newPageNum = {[name] : pageNum };
            const fieldPageNums = {...oldPageNums, ...newPageNum};
            this.setState((state, props) => ({fieldPageNums}));
            console.log("page num is now" + pageNum);
            console.log(JSON.stringify(this.state.fieldPageNums));
        };

        const forward = () => {
            console.log("clicked the forward button!");
            set_page_num(pageNum + 1);
        };

        const back = () => {
            console.log("clicked the forward button!");
            set_page_num(pageNum - 1);
        };

        const jump = (targetPageNum) => () => set_page_num(targetPageNum);

        const pagers = {forward, back, jump};

        return {name,
                contents,
                num_pages,
                current_page : pageNum,
                pagers
               };
    }
};

export const FieldPaginator = new FieldPaginatorClass();


export default LongListBrowser;
