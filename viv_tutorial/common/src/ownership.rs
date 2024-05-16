pub fn ownership_a() {

    //--> [ Stack (스택) ]
    // 스택에 값을 푸시 하는 것을 할당이라고 하지 않고, 바인딩이라는 표현을 사용함.

    //--> [ Heap ]
    // 1. 운영체제에게 저장할 공간이 있는지 물어보는 단계
    // 2. 할당 (allocating on the heap) : 할당자 힙 영역 안에 어떤 빈 지점을 찾아,
    // 3. 이지점은 사용중이라고 표시한 뒤 해당 지점을 가리키는 포인터를 우리한테 반환함.
    // 4. 반환된 해당 포인터는 크기가 정해져 있어 스택에 저장할 수 있음.
    // 5. 힙 구조는 손님이 레스토랑에서 자리를 잡고 않는 과정과 비교할 수 있음.

    // --> 레스토랑(메모리)에 입장
    // --> 직원(포인터변수, 스택에 존재)에게 인원수를 알림
    // --> 직원은 인원수에 맞는 빈 테이블(힙 메모리 할당)을 찾아 안내한 후 직원은 해당 자리위치를 기억하고 있음.
    // --> 이후에 온 일행이 우리 테이블(힙 메모리 주소)을 찾을 땐 직원(스택의 포인터 변수)에게 물어 안내를 받음(테이블로 찾아감, 실제 힙 메모리 영역).

    // --> [ 소유권 ]
    // 각 값은 소유자 (owner) 가 정해져 있음.
    // 한 값의 소유자는 동시에 여럿 존재할 수 없음.
    // 소유자가 스코프 밖으로 벗어날 때, 값은 버려짐 (dropped)

    {
        // s 는 아직 선언되지 않아서 여기서는 유효하지 않음.
        let s = "hi"; // 이 지점 부터 s 가 유효함.
                      // s로 어떤 작업
        println!("\u{26EC} s = {}", s);
    } // 이 스코프가 종료되었고, s 가 더 이상 유효하지 않음.

    {
        // --> [ String ]
        // 1. 힙에 할당된 데이터를 다루기 때문에, 컴파일 타임에 크기를 알 수 없는 텍스트를 저장할 수 있음.
        // 2. 변경가능
        // 3. 실행 중 메모리 할당자로 부터 메모리를 요청해야 함.
        // 4. String 사용을 마쳤을 때 메모리를 해제할 (할당자에게 메모리를 반납할) 방법이 필요함.
        // 5. GC (garbage collector, 가비지 컬렉터)
        // 6. 러스트는 소속된 스코프를 벗어나는 순간 자동으로 메모리를 해제하는 방식으로 GC 처리함.

        let s1 = String::from("Hello, World"); // 3번단계, 메모리 요청
        println!("\u{26EC} &s1 {:p}", &s1);
        let s2 = s1; // (move) s2 로 move 되고 삭제됨. (만일 동시 존재하게 되면 -> 중복 해제 금지위반 , double free error)

        // println!("\u{26EC} {}", s1); // s1 은 삭제되었으므로 유효하지 않음.
        println!("\u{26EC} &s2 {:p}", &s2); // 유효함.

        // 러스트는 깊은 복사로 데이터를 복사하는 일은 없음.

        // 깊은 복사.
        let s3 = s2.clone();
        println!(
            "\u{26EC} s3 = {}, s2 = {} = &s3 {:p}, &s2 {:p}",
            s3, s2, &s3, &s2
        );

        // stack example : 크기가 고정되어 모두 스택에 저장되기 때문에 정수형은 정상작동함.
        let a = 12;
        let b = a;
        println!("\u{26EC} &a = {:p}, &b = {:p}", &a, &b);

        // --> Copy 가 가능한 타입
        // 모든 정수형
        // true, false, bool
        // 부동소수점
        // Copy 가능한 타입만으로 구성된 튜플 : (i32, i32)

        // --> 함수로 값을 전달
        //     함수로 값을 전달 하는 메커니즘은 변수에 값을 대입할 때와 유사함.

        //     이동 또는 복사
        // (이동, move)
        let c = String::from("some string");
        println!("\u{26EC} {}, {:p}", c, &c);
        takes_ownership(c); // s의 값이 함수로 이동됨.
                            // ... 여기서는 c 가 더 이상 유효하지 않음.

        // (복사, copy)
        let d = 5;
        makes_copy(d); // i32 는 Copy 이므로 계속 d 를 사용할 수 있음.
        println!("\u{26EC} Scope: {}", d);

        // --> 함수 반환값
        let r1 = gives_ownership();
        println!("\u{26EC} r1 = {}", r1);
        let r2 = String::from("Fine Thanks");
        let r3 = takes_and_gives_back(r2);
        println!("\u{26EC} r3 = {}, r2 = disposed", r3);
    }

    {
        let s1 = String::from("vivakr");
        let (s2, len) = calculate_length(s1);
        println!("\u{26EC} The length of '{}' is {}.", s2, len);
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_ownership(some_string: String) {
    println!("\u{26EC} {}, {:p}", some_string, &some_string);
}

fn makes_copy(some_integer: i32) {
    println!("\u{26EC} make_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 호출자 함수 쪽으로 이동함.
}

pub fn first_wrod(s: &String) -> &str {
    // &str : 문자열 슬라이스
    // as_bytes() : 바이트 배열로 변환.
    let bytes = s.as_bytes();

    // iter() : 컬렉션의 각 요소 반환
    // enumerate() : iter 의 각 결과값을 튜플로 감싸 반환 (인덱스, 해당요소의 참조자)
    let tuple = bytes.iter().enumerate();

    for (i, &item) in tuple {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn first_wrod_ref(s: &str) -> &str {
    &s[..5]
}
