import React from 'react'
import NavItem from './nav-item'

const NavBar = () => (
  <nav className="nav-menu">
    <ul className="nav-list">
      <NavItem path="/" name="Home" />

      {/*<NavItem path="/shows" name="Shows" />*/}

      <NavItem path="/shows/add" name="Add Show" />
    </ul>
  </nav>
)

export default NavBar
