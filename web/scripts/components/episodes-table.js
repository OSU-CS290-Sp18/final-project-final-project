import React from 'react'
import EpisodesTableRow from './episodes-table-row'

const EpisodesTable = ({ episodes }) => {
  const rows = [].concat(episodes)
    .sort((a, b) => {
      if (a.num === null && b.num !== null) {
        return 1
      } else if (b.num === null && a.num !== null) {
        return -1
      } else if (b.num === null && a.num == null) {
        return -1
      }

      return a.num > b.num
    })
    .map(ep =>
      <EpisodesTableRow key={ep.id} episode={ep} />
    )

  return (
    <table className="episodes-table">
      <tbody>
        {rows}
      </tbody>
    </table>
  )
}

export default EpisodesTable
