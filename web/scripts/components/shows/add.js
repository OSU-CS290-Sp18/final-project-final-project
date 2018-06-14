import React from 'react'
import { stringify as QStringify } from 'qs'
import ShowsTable from '../shows-table'

export default class ShowsAdd extends React.Component {
  constructor(props) {
    super(props)
    this.state = {
      searching: false,
      results: []
    }
    //this.onSearch = debounce(this.onSearch, 200)
    this.onSearch = this.onSearch.bind(this)
  }

  onSearch(e) {
    e.persist()

    this.setState({
      searching: true,
      results: []
    })

    const that = this;
    const query = QStringify({ q: e.target.value });

    fetch(`/api/search?${query}`)
      .then(res => res.json())
      .then(json => {
        that.setState({
          searching: false,
          results: json
        })
      })
  }

  render() {
    let results

    if (this.state.searching) {
      results = (
        <section className="show-search-results">
          <h3 className="default-subtitle">Searching...</h3>
        </section>
      )
    } else if (!this.state.searching && this.state.results.length > 0) {
      results = (
        <section className="show-search-results">
          <h3 className="default-subtitle">Search Results</h3>
          <ShowsTable shows={this.state.results} addShow={true} />
        </section>
      )
    } else {
      results = (
        <section className="show-search-results">
          <h3 className="default-subtitle">No Results</h3>
        </section>
      )
    }

    return (
      <main className="main main-default">
        <h2 className="default-page-title">Add Show</h2>

        <div className="show-search-area">
          <input className="show-search-box" type="text" name="showName" placeholder="Search" onChange={this.onSearch} />
        </div>

        {results}
      </main>
    )
  }
}

// http://davidwalsh.name/javascript-debounce-function
function debounce(func, wait, immediate) {
	var timeout;
	return function() {
		var context = this, args = arguments;
		var later = function() {
			timeout = null;
			if (!immediate) func.apply(context, args);
		};
		var callNow = immediate && !timeout;
		clearTimeout(timeout);
		timeout = setTimeout(later, wait);
		if (callNow) func.apply(context, args);
	};
};
