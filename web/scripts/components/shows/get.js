import React from 'react'
import EpisodesTable from './../episodes-table'

export default class ShowsGet extends React.Component {
  constructor(props) {
    super(props)
    this.state = {
      loading: true,
      show: {
        seasons: []
      }
    }
  }

  componentWillMount() {
    const that = this;
    fetch(`http://localhost:8000/api/shows/${this.props.match.params.id}`)
      .then(res => res.json())
      .then(json => {
        that.setState({
          loading: false,
          show: json
        })
      })
  }

  render() {
    const title = this.state.loading ? "Loading..." : this.state.show.name
    let episodes = 0
    let episodesTables = []

    if (!this.state.loading) {
      for (let i = 0; i < this.state.show.seasons.length; i++) {
        for (let j = 0; j < this.state.show.seasons[i].episodes.length; j++) {
          episodes++
        }
      }

      episodesTables = [].concat(this.state.show.seasons)
        .sort((a, b) => a.num > b.num)
        .map(season => {
          return (
            <section key={season.id} className="show-season show-episodes-list">
              <div className="show-season-titlebar">
                <h3 className="default-subtitle">Season {season.num} Episodes</h3>
                <span className="season-titlebar-ep-cnt"><em>{season.episodes.length}</em> Episodes</span>
              </div>
              <EpisodesTable episodes={season.episodes} />
            </section>
          )
        })
    }

    return (
      <main className="main main-default">
        <h2 className="default-page-title">{title}</h2>

        <div className="show-info">
          <div className="show-banner">
            <img className="show-cover-img" src={this.state.show.cover_img} />
          </div>

          <div className="show-metadata">
            <section className="show-summary-section">
              <h4 className="default-subsubtitle">Summary</h4>
              <p className="show-summary">{this.state.show.summary}</p>
            </section>

            <section className="show-stats-section">
              <h4 className="default-subsubtitle">Statistics</h4>
              <div className="show-stat">
                <em className="show-stat-name">Seasons</em>:
                <span className="show-stat-value"> {this.state.show.seasons.length}</span>
              </div>

              <div className="show-stat">
                <em className="show-stat-name">Episodes</em>: 
                <span className="show-stat-value"> {episodes}</span>
              </div>
            </section>
          </div>
        </div>

        {episodesTables}
      </main>
    )
  }
}
