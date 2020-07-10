class ArticleForm extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            title: '',
            content: ''
        };

        this.handleChange = this.handleChange.bind(this);
        // this.handleInputChange = this.handleInputChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }



    // https://stackoverflow.com/a/25187443/3536879
    handleChange(key) {
        return function (e) {
            this.setState({[key] : e.target.value});
        }.bind(this);
    }

    handleSubmit(event) {

        console.log("posted "+this.state);
        axios.post(API_URL+ENDPOINT_URL, {
            title: this.state.title,
            content: this.state.content
        });        

    }

    render() {
        return (
            <form onSubmit={this.handleSubmit}>
              Title:
              <input
                value={this.state.title}
                onChange={this.handleChange('title')}
              />
              <br/>
              Content:<br/>
              <input
                value={this.state.content}
                onChange={this.handleChange('content')}
              />
              <button type="submit"/>
            </form>
        );
    }
}

export default ArticleForm;
