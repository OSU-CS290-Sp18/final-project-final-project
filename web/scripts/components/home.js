import React from 'react'
import ShowsTable from './shows-table'

export default class Home extends React.Component {
  constructor(props) {
    super(props)
    this.state = {
      shows: []
    }
  }

  componentWillMount() {
    const that = this;
    fetch('/api/shows')
      .then(res => res.json())
      .then(json => {
        that.setState({
          shows: json
        })
      })
  }

  render() {
    const shows = [].concat(this.state.shows)
      .sort((a, b) => a.name > b.name)

    return (
      <main className="main main-default">
        <h2 className="default-page-title">Your Shows</h2>
        <ShowsTable shows={shows} />
      </main>
    )
  }
}
