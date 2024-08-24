# BroHowDoI

# Build Instructions
Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` on your **Linux** terminal. This should install the Rust toolchain on your system.
Then run `git clone https://github.com/nescafe-abuser/bisleri.git --depth 1 ~/BroHowDoI && cd BroHowDoI && cargo build`. This will build the binary.
To run the server and serve the website at localhost, run `cargo run`. After the prompt says that the server is online, point your browser to http://127.0.0.1:8000. You should see the website there.
If it says port blocked, check your firewall and unblock the port.
To host the website, put all your files in the static directory and name the home page file as `index.html`. That's all.

# Overview
**Instructors** create pages where they can:
1. Detail their course
2. Provide an interactive roadmap
3. Detail their past achievements, goals and qualifications
4. Advertise other services and social media
5. Network/Collaborate with other instructors

**Learners** can then visit pages and subscribe to a course.

Pages are found by searching for topics, tags, instructors, discipline or skill level.

## Stack

Backend:
1. Rust
2. ??? (NGINX??)

Frontend:
1. ??? (React??)

## Objective

Make a platform that lets instructors offer courses to large audiences with a simple UI for usability and rock solid service.
For learners, courses can be consumed in roadmaps, playlists or freeform.
This platform will be un-opinionated and no instructor should have to bother about any "recommendation algorithm" to compete. Ratings should be the sole focus.
To keep learners hooked, their achievements and activities can be interfaced with social media to show off. They can also unlock direct rewards designed by their favourite instructors.
To reward instructors, milestones will be put in place.

## Expected Features

1. [x] Web server in Rust
2. [ ] Rating system for pages
3. [ ] Sign in, sign up for instructors and learners
4. [ ] Basic dashboard for instructors
5. [ ] Roadmap
6. [ ] Content hosting
7. [ ] Search by rating, instructor, discipline, topics, tags and skill level

# Lead Developers

**Maharshi Mukherjee**[23053682@kiit.ac.in]
**Suhani Kothari**[22051290@kiit.ac.in]
**Anik Mandal**[2205615@kiit.ac.in]
