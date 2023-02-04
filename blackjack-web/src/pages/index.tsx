import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import styles from '@blackjack-rs/styles/Home.module.css'
import init, { Blackjack } from '../../../blackjack-rs/pkg/blackjack_rs.js'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  init().then(() => {
    const game = Blackjack.init();

  })
  
  return (
    <>
    
    </>
  )
}
