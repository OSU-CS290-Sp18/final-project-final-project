import React from 'react'
import ShowsTableRow from './shows-table-row'

const ShowsTable = (params) => {
  const rows = params.shows.map(show =>
    <ShowsTableRow key={show.id} show={show} addShow={params.addShow} />
  )

  return (
    <table className="shows-table">
      <tbody>
        {rows}
      </tbody>
    </table>
  )
}

export default ShowsTable
