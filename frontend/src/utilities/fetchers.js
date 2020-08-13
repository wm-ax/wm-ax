// import qs from 'qs';

import {settings, api} from '../settings.js';

const API_URL = settings.API_URL;

let FetcherClass = class {

    fetchData(query_data = {})  {
        const rel_path = this.props.location.pathname;
        const qs = this.props.location.search;
        const query_string = (new URLSearchParams(qs)).toString();
        const url = `${API_URL}${rel_path}?${query_string}`;
        api.get(url)
            .then(result => this.setState(result.data))
            .catch(error => this.setState({ error }));
    }

    fetchMoreData(field, chunk_size) {
        const rel_path_raw = this.props.location.pathname;
        const rel_path = rel_path_raw === "/" ? "" : rel_path_raw;
        const qs = this.props.location.search;
        const existing_query = (new URLSearchParams(qs)).toString();
        const lo = this.state[field].length;
        const page_query = `field=${field}&from=${lo}&num=${chunk_size}`;
        const extend = new_data => (state, props) =>
              ({[field] : [...this.state[field],
                           ...new_data]});
        const url = `${API_URL}${rel_path}/more?${existing_query}&${page_query}`;
        api.get(url)
            .then(result => this.setState(extend(result['data'][field])))
            .catch(error => this.setState({ error }));
    }

}


const Fetchers = new FetcherClass();

export default Fetchers;
