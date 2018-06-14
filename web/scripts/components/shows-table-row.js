import React from 'react'
import { Link, Redirect } from 'react-router-dom'
import { stringify as QStringify } from 'qs'

export default class ShowsTableRow extends React.Component {
  constructor(props) {
    super(props)
    this.state = {
      waiting: false,
      redirect: false,
      id: -1
    }
  }

  addShow(e) {
    if (this.state.waiting) {
      return
    } else {
      this.setState({
        waiting: true,
        redirect: false,
        id: -1
      })
    }

    const that = this;

    fetch('http://localhost:8000/api/shows', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded; charset=UTF-8'
      },
      body: QStringify({
        id: this.props.show.provider_id
      })
    })
    .then(res => res.json())
    .then(json => {
      that.setState({
        waiting: false,
        redirect: true,
        path: `/show/${json.id}`
      })
    })
  }

  render() {
    if (this.state.redirect) {
      return <Redirect push to={this.state.path} />
    }

    const show = this.props.show;
    const path = `/show/${show.id}`;
    let episodes = 0;
    const seasonsCount = show.seasons === null ? 0 : show.seasons.length;

    if (show.seasons !== null) {
      for (let i = 0; i < show.seasons.length; i++) {
        for (let j = 0; j < show.seasons[i].episodes.length; j++) {
          episodes++;
        }
      }
    }

    let imgLink, nameLink

    if (this.props.addShow === true) {
      imgLink = (
        <a href="#" onClick={this.addShow.bind(this)} className="shows-table-img-link">
          <img className="shows-table-img" src={show.cover_img} />
        </a>
      )

      nameLink = (
        <a href="#" onClick={this.addShow.bind(this)} className="shows-table-img-link">
          {show.name}
        </a>
      )
    } else {
      imgLink = (
        <Link to={path} className="shows-table-img-link">
          <img className="shows-table-img" src={show.cover_img} />
        </Link>
      )

      nameLink = (
        <Link to={path} className="shows-table-link">{show.name}</Link>
      )
    }

    return (
      <tr className="shows-table-row">
        <td className="show-img-col">
          {imgLink}
        </td>
        <td>
          <div className="shows-table-metadata-col">
            <h3 className="shows-table-title">
              {nameLink}
            </h3>
            
            <p className="shows-table-summary">{show.summary}</p>
          </div>
        </td>
      {/*<td>
          <span className="shows-table-cnt seasons">{seasonsCount}</span>
        </td>
        <td>
          <span className="shows-table-cnt episodes">{episodes}</span>
        </td>*/}
      </tr>
    )
  }
}
