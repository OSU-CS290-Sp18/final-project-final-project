import React from 'react'

const EpisodesTableRow = ({ episode }) => {
  const num = episode.num === null ? 'SP' : episode.num

  return (
    <tr className="episodes-table-row">
      <td>{num}</td>
      <td>{episode.name}</td>
    </tr>
  )
}

export default EpisodesTableRow
