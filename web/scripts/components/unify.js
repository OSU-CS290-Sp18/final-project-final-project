import React from 'react'
import { Provider } from 'react-redux'
import {
  BrowserRouter as Router,
  Route
} from 'react-router-dom'
import Header from './header'
import Home from './home'
import ShowsIndex from './shows/index'
import ShowsAdd from './shows/add'
import ShowsGet from './shows/get'

const Unify = () => (
  <Router>
    <div>
      <Header />

      <Route exact path="/" component={Home} />
      {/*<Route exact path="/shows" component={ShowsIndex} />*/}
      <Route exact path="/shows/add" component={ShowsAdd} />
      <Route exact path="/show/:id" component={ShowsGet} />
    </div>
  </Router>
)

export default Unify
