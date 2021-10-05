with import <nixpkgs>{};

rustPlatform.buildRustPackage rec {
  pname = "copie";
  version = "0.2.1";

  src = fetchFromGitHub {
    owner = "JonathanxD";
    repo = pname;
    rev = "${version}";
    sha256 = "1fdbs5d03wpn8djbkf95hdhyl14pd1v4vwq1szh8izbbv0dx44kv";
  };

  cargoSha256 = "0i60afizkj2skb3jx65pwwzkgrvbnp0177vww1imk33px5x3p13h";

  meta = with lib; {
    description = "copy with environment variables";
    homepage = "https://github.com/JonathanxD/copie";
    license = licenses.mit;
    maintainers = with maintainers; [
      {
        name = "Jonathan H. R. Lopes";
        github = "JonathanxD";
        githubId = 5360060;
        matrix = "@jonathanhrl:matrix.org";
        email = "joniweb01@gmail.com";
        keys = [{
            longkeyid = "rsa3072/0x4DF5FC43FD4FE9CC";
            fingerprint = "B2C2 7303 C091 E72F C62A  AB69 4DF5 FC43 FD4F E9CC";
        }];
      }
    ];
  };
}
